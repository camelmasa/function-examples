import "wasi";
import { Console } from "as-wasi/assembly";
import { JSON } from "assemblyscript-json/assembly"; 

const input = Console.readAll();
const _config = <JSON.Obj>(JSON.parse(input));

Console.log(`{
  "discounts": [],
  "discountApplicationStrategy": "FIRST"
}`);