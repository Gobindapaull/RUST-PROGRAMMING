use hyper::{ Client, Uri};

#[tokio::main]
async fn main() {
    let client = Client::new(); // to create a new http client
    let uri = "http://www.google.com".parse::<Uri>().unwrap(); // turbo fish syntax with type Uri from hyper.
    let result = client.get(uri).await.unwrap();
    // get method on client to sent a request to the uri
    println!("{:?}", result.status());
}
