import React, { Suspense } from "react";
import { createGlobalStyle } from "styled-components";

import Editor from "./components/Editor";
import Viewport from "./components/Viewport";
import typeface from "./styles/typeface";

const GlobalStyles = createGlobalStyle`
  body {
    background-color: #9944ff;
    color: #ffffff;
    font-size: 16px;
    margin: 0;
    overflow: hidden;
    padding: 0;
    ${typeface}
  }
`;

export default function App(): JSX.Element {
  return (
    <Suspense fallback={<Viewport title="loading..." />}>
      <GlobalStyles />
      <Viewport title="todo.txt">
        <Editor />
      </Viewport>
    </Suspense>
  );
}
