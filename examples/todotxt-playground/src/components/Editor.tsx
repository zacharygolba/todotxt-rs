import React, { lazy, useState } from "react";
import AutosizeTextarea from "react-autosize-textarea";
import styled from "styled-components";

import typeface from "../styles/typeface";

const Code = styled.code`
  color: rgba(255, 255, 255, 0.4);
  white-space: pre-wrap;
  ${typeface}
`;

const Panel = styled.section`
  flex-shrink: 0;
  overflow-x: hidden;
  overflow-y: auto;
  width: calc(50% - 20px);

  & ~ & {
    margin-left: 20px;
    width: 50%;
  }

  & > textarea {
    background-color: transparent;
    border-style: none;
    color: #ffffff;
    font-size: 1rem;
    line-height: 1.5rem;
    min-height: 100%;
    outline: none;
    resize: none;
    width: 100%;
    ${typeface}
  }
`;

type EditorProps = {
  parse(input: string): string;
};

function Editor(props: EditorProps): JSX.Element {
  const [value, setValue] = useState("");

  return (
    <>
      <Panel>
        <AutosizeTextarea
          async
          autoCapitalize="off"
          autoFocus={true}
          defaultValue={value}
          onChange={({ target }: any) => setValue(target.value)}
          placeholder="(A) enter a task here @example +todo.txt"
          rows={3}
          spellCheck={false}
        />
      </Panel>
      <Panel>
        <Code>{props.parse(value)}</Code>
      </Panel>
    </>
  );
}

export default lazy(async () => {
  const { parse } = await import("../../rust/pkg");

  return {
    default: () => <Editor parse={parse} />
  };
});
