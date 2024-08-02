# v1.0.0
- Stable semver version
- Add deno build
- Add better documentation and github page playground
- Update dict from OpenCC
- support exports field in package.json

Instead of
```javascript
import init, {simplecc} from 'simplecc-wasm/pkg/web/simplecc_wasm';
```
Use 
```typescript
import init, {simplecc} from 'simplecc-wasm';
```

More info on exports field https://nodejs.org/api/packages.html#community-conditions-definitions and typescript [customCondition config](https://nodejs.org/api/packages.html#community-conditions-definitions)
