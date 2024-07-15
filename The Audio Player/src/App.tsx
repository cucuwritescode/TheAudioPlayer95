import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import styled, { createGlobalStyle, ThemeProvider } from "styled-components";
import { styleReset, Button, Window, WindowHeader, WindowContent, TextField } from "react95";
import original from "react95/dist/themes/original";
import "./App.css";

const GlobalStyles = createGlobalStyle`
  ${styleReset}
  body {
    font-family: 'ms_sans_serif';
  }
`;

const Container = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  background-color: #008080;
`;

const App: React.FC = () => {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <ThemeProvider theme={original}>
      <GlobalStyles />
      <Container>
        <Window>
          <WindowHeader>
            <span>the-audio-player.exe</span>
          </WindowHeader>
          <WindowContent>
            <h1>Welcome to Tauri!</h1>
            <div className="row">
              <img src="/vite.svg" className="logo vite" alt="Vite logo" />
              <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
              <img src="/react.svg" className="logo react" alt="React logo" />
            </div>
            <p>Click on the logos to learn more.</p>
            <form
              className="row"
              onSubmit={(e) => {
                e.preventDefault();
                greet();
              }}
            >
              <TextField
                fullWidth
                placeholder="Enter a name..."
                onChange={(e) => setName(e.target.value)}
                value={name}
              />
              <Button type="submit">Greet</Button>
            </form>
            <p>{greetMsg}</p>
          </WindowContent>
        </Window>
      </Container>
    </ThemeProvider>
  );
};

export default App;
