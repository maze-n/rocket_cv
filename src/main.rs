use std::path::Path;

#[macro_use]
extern crate rocket;
use rocket::form::Form;
use rocket::fs::NamedFile;

mod forms;

use forms::{BlurForm, ResizeForm};

use opencv::{
    core::*,
    imgcodecs::{imread, imwrite},
    imgproc::{gaussian_blur, resize, INTER_CUBIC},
};

#[post(
    "/change-size/<x>/<y>",
    format = "multipart/form-data",
    data = "<form>"
)]
async fn change_size(x: f64, y: f64, mut form: Form<ResizeForm<'_>>) -> Option<NamedFile> {
    form.image.persist_to("files/src.jpg").await.unwrap();
    let src_img = imread("files/src.jpg", -1).unwrap();
    let mut dest_img = Mat::default();

    resize(
        &src_img,
        &mut dest_img,
        Size2i::new(0, 0),
        x,
        y,
        INTER_CUBIC,
    )
    .unwrap();

    imwrite(
        "files/result.jpg",
        &dest_img,
        &opencv::core::Vector::default(),
    )
    .unwrap();

    NamedFile::open(Path::new("files/").join("result.jpg"))
        .await
        .ok()
}

#[post("/blur", format = "multipart/form-data", data = "<form>")]
async fn image_blur(mut form: Form<BlurForm<'_>>) -> Option<NamedFile> {
    form.image.persist_to("files/src.jpg").await.unwrap();
    let src_img = imread("files/src.jpg", -1).unwrap();
    let mut dest_img = Mat::default();

    gaussian_blur(
        &src_img,
        &mut dest_img,
        Size::new(form.ksize_height, form.ksize_width),
        form.sigma_x,
        form.sigma_y,
        BORDER_DEFAULT,
    )
    .unwrap();

    imwrite(
        "files/result.jpg",
        &dest_img,
        &opencv::core::Vector::default(),
    )
    .unwrap();

    NamedFile::open(Path::new("files/").join("result.jpg"))
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![change_size, image_blur])
}
