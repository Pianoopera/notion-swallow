# notion-swallow cheat sheet

## Pages

### Generate curl to get page information

```sh
notion-swallow pages -i {page_id}
```

### Generate curl to create page

```sh
notion-swallow pages -x POST -f ./fixture/post/create_request1.json
```

### Generate crul to get property information in page

```sh
notion-swallow property_pages -i {page_id} -p Name
```

### Generation curl to update properties in page

```sh
notion-swallow pages -x PATCH -i {page_id} -f ./fixture/patch/pages_patch_request1.json
```

## Databases

### Generate curl to create database

```sh
notion-swallow databases -x POST -f ./fixture/post/create_database1.json
```

### Generate curl to retrieve records in database with specified conditions

```sh
notion-swallow query_databases -i {database_id} -f ./fixture/post/request1.json
```

### Generate curl to retrieve database information

```sh
notion-swallow databases -i {database_id}
```

### Generate curl to update information in database

```sh
notion-swallow databases -x PATCH -i {database_id} -f ./fixture/patch/request1.json
```

## Blocks

### Generate curl to add block to page

```sh
notion-swallow append_blocks -x PATCH -i {page_id} -f ./fixture/patch/append_blocks_request1.json
```

### Generate curl to add block under block

```sh
notion-swallow append_blocks -x PATCH -i {block_id} -f ./fixture/patch/append_blocks_request1.json
```

### Generate curl to get block information

```sh
notion-swallow blocks -i {block_id}
```

### Generate curl to get information under block

```sh
notion-swallow children_blocks -i {block_id}
```

### Generate a curl to update a block

```sh
notion-swallow blocks -x PATCH -i {block_id} -f ./fixture/patch/blocks_update_request1.json
```

### Delete a block

```sh
notion-swallow blocks -x DELETE -i {block_id}
```

## Search

### Generate curl to search with specified information

```sh
notion-swallow search -f ./fixture/post/search_by_title_request1.json
```