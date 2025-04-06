- カラム追加
    - `cargo loco g migration AddDescriptionToMovies description:string`
    - https://loco.rs/docs/the-app/models/
    - Add{Xxx}To{テーブル名} {カラム名}:{型}

- モデル作成
    - `cargo loco g model review movie_id:small_int  rating:small_int comment:string!`
- コントローラー作成
    - `cargo loco generate controller review --kind html`