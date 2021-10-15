import { string, rest, whitespace, newline } from "parjs";
import { then, maybe, qthen, thenq, between, or } from "parjs/combinators";

const idParser = string("id: myButton_Symbol").pipe(between(whitespace(), whitespace().pipe(or(newline()))));

const itemBodyParser = idParser
  .pipe(between(string("{"), string("}")))

const itemParser = string("Item")
  .pipe(then(whitespace()))
  .pipe(maybe(' '))
  .pipe(then(itemBodyParser))


export default itemParser;