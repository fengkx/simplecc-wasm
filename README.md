# simplecc-wasm

> Simple OpenCC alternative with wasm based on [simplecc-rs](https://github.com/sorz/simplecc-rs)

# Usage

```js
import { simplecc } from "simplecc-wasm";
simplecc("发财了去植发", "s2t"); // '發財了去植髮'
```

## 配置 Configurations

[Opencc 配置中的一部分](https://github.com/BYVoid/OpenCC#configurations-%E9%85%8D%E7%BD%AE%E6%96%87%E4%BB%B6)

```plain
"s2t"
"t2s"
"s2tw"
"s2hk"
"s2twp"
"hk2s"
```
