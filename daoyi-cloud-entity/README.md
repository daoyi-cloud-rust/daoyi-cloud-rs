```shell
sea-orm-cli generate entity --with-serde both --model-extra-attributes 'serde(rename_all="camelCase")' --date-time-crate chrono -o src/entity/demo
```

```rust
use daoyi_cloud_entity::entity::demo::prelude::*;
use sea_orm::prelude::*;
```