use serde::Deserialize;

#[tokio::main]
async fn main() {
    let image = reqwest::get("https://dog.ceo/api/breeds/image/random").await.unwrap().json::<ImageResponse>().await.unwrap();

    println!("{:?}", image);
}

#[derive(Deserialize, Debug)]
struct ImageResponse {
    message: String,
    status: String
}
// ImageResponse { message: "https://images.dog.ceo/breeds/cavapoo/doggo1.jpg", status: "success" }
