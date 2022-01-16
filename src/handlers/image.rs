use uuid::Uuid;
use bytes::BufMut;
use futures::TryStreamExt;
use warp::{Filter, Reply, Rejection, multipart::{FormData, Part}};

pub fn images_filter()-> impl Filter<Extract= impl Reply, Error= Rejection> + Clone{
  warp::path("image")
    .and(warp::path("upload"))
    .and(warp::post())
    .and(warp::multipart::form())
    .and_then(upload_image)
}

async fn upload_image(form: FormData)-> Result<impl Reply,Rejection>{
  let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
    eprintln!("form error: {}", e);
    warp::reject::reject()
  })?;

  for p in parts {
      if p.name() == "file" {
          let content_type = p.content_type();
          let file_ending;
          match content_type {
              Some(file_type) => match file_type {
                  "application/pdf" => {
                      file_ending = "pdf";
                  }
                  "image/png" => {
                      file_ending = "png";
                  }
                  v => {
                      eprintln!("invalid file type found: {}", v);
                      return Err(warp::reject::reject());
                  }
              },
              None => {
                  eprintln!("file type could not be determined");
                  return Err(warp::reject::reject());
              }
          }

          let value = p
              .stream()
              .try_fold(Vec::new(), |mut vec, data| {
                  vec.put(data);
                  async move { Ok(vec) }
              })
              .await
              .map_err(|e| {
                  eprintln!("reading file error: {}", e);
                  warp::reject::reject()
              })?;

          let file_name = format!("./files/{}.{}", Uuid::new_v4().to_string(), file_ending);
          tokio::fs::write(&file_name, value).await.map_err(|e| {
              eprint!("error writing file: {}", e);
              warp::reject::reject()
          })?;
          println!("created file: {}", file_name);
      }
  }
  Ok("success")
}