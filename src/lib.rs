use graphul::{http::Methods, Context, Graphul};
use serde_json::json;
use sync_wrapper::SyncWrapper;

#[tokio::main]
async fn graphul() -> shuttle_service::ShuttleAxum {
    let mut app = Graphul::new();

    let v = json!({
      "name": "Juan",
      "age": 20,
      "address": {
        "street": "Ferris",
        "number_house": 4356
      }
    });

    app.get("/", || async { "Hola Mundo" });

    app.get("/json", |c: Context| async move { c.json(v) });
    let sync_wrapper = SyncWrapper::new(app.export_routes());

    Ok(sync_wrapper)
}
