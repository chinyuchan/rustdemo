# Get FRA balance of address from csv file

## 编译
```
cargo build --release
```

## 运行
```
./target/release/readcsv --file your_file.csv --url rpc_url
```
* 输入文件的标头为：`"id","ticker","user","amount","price","state","to_user","center_mnemonic","create_time","update_time","center_user"`即`list_record`表的结构
* 根据`center_mnemonic`，余额大于0的地址会写入到`output_your_file.csv`文件中


