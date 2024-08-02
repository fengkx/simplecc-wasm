import { Flex, RadioGroup, TextArea, Text } from "@radix-ui/themes";
import { use, useDeferredValue, useMemo, useState } from "react";
import init, { simplecc } from "simplecc-wasm";
const configurations = ["s2t", "t2s", "s2tw", "s2hk", "s2twp", "hk2s"] as const;

const wasmInit = init();

export function Demo() {
  use(wasmInit);
  const [text, setText] = useState("发财了去植发");
  const [config, setConfig] = useState<(typeof configurations)[number]>("s2t");
  const converted = useMemo(() => {
    return simplecc(text, config);
  }, [text, config]);
  const deferredResult = useDeferredValue(converted);
  return (
    <Flex gap="4" direction="column">
      <Text weight={"bold"}>原文：</Text>
      <TextArea
        value={text}
        onChange={(ev) => {
          setText(ev.target.value);
        }}
        resize="both"
        placeholder="输入文字进行转换"
      />
      <Text weight={"bold"}>配置：</Text>
      <Flex align="start" direction="column" gap="1">
        <RadioGroup.Root
          size="3"
          value={config}
          onValueChange={(val) => {
            // @ts-expect-error
            setConfig(val);
          }}
        >
          <Flex direction="column" gap="2">
            {configurations.map((conf) => {
              return (
                <RadioGroup.Item key={conf} value={conf}>
                  {conf}
                </RadioGroup.Item>
              );
            })}
          </Flex>
        </RadioGroup.Root>
      </Flex>
      <Text weight={"bold"}>结果：</Text>
      <TextArea variant="soft" readOnly value={converted} />
    </Flex>
  );
}
