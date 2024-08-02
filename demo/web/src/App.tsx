import {
  Flex,
  TextArea,
  Text,
  Radio,
  RadioGroup,
  Container,
} from "@radix-ui/themes";
import React from "react";
import { Demo } from "./demo";

import "./App.css";

const App = () => {
  return (
    <Container className="container">
      <Demo />
    </Container>
  );
};

export default App;
