use trpl::{Html};
use std::pin::Pin;
use std::process;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        panic!("Please provide at least two sites");
    }
    let mut it = args.iter();
    it.next();

    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let mut futures = Vec::<Pin<Box<dyn Future<Output = ()>>>>::new();
        for url in it {
            let tx_clone = tx.clone();
            let tx_fut = async move {
                let start = Instant::now();
                let text = trpl::get(url).await.text().await;
                let title = Html::parse(&text)
                    .select_first("title")
                    .map(|title| title.inner_html());
                let end = start.elapsed();
                tx_clone.send((url, title, end)).unwrap();
            };

            futures.push(Box::pin(tx_fut));
        }

        drop(tx);
        trpl::join_all(futures).await;

        while let (url, maybe_title, t) = rx.recv().await.unwrap_or_else(|| {
            process::exit(0);
        }) { 
            let millis = t.as_millis();
            println!("{url} returned");
            println!("It took {millis} milliseconds");
            match maybe_title {
                Some(title) => println!("Its page title is: '{title}'"),
                None => println!("Its title could not be parsed."),
            }

        }
    })
}
