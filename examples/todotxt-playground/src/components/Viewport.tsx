import React, { PropsWithChildren } from "react";
import styled from "styled-components";

type ViewportProps = PropsWithChildren<{
  title?: string;
}>;

const Content = styled.main`
  display: flex;
  height: calc(100% - 100px);
`;

const Screen = styled.div`
  background-color: #15161d;
  border-radius: 4px;
  overflow: hidden;
  padding: 0 20px;
  position: absolute;
  top: 8px;
  bottom: 8px;
  left: 8px;
  right: 8px;
`;

const Title = styled.header`
  font-size: 2rem;
  line-height: 5rem;
  user-select: none;
`;

export default function Viewport(props: ViewportProps): JSX.Element {
  return (
    <Screen>
      <Title>{props.title}</Title>
      <Content>{props.children}</Content>
    </Screen>
  );
}
