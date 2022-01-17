use rocket::fs::TempFile;

#[derive(FromForm)]
pub struct ResizeForm<'f> {
    pub image: TempFile<'f>,
}

#[derive(FromForm)]
pub struct BlurForm<'f> {
    pub image: TempFile<'f>,
    pub ksize_height: i32,
    pub ksize_width: i32,
    pub sigma_x: f64,
    pub sigma_y: f64,
}
