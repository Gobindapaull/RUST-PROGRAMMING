use hyper::{ Client, Uri};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let uri = "http://www.google.com".parse::<Uri>().unwrap();
    let result = client.get(uri).await.unwrap();
    println!("{:?}", result.status());
}
