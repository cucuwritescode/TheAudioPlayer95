import React, { useState, useRef } from "react";
import styled, { createGlobalStyle, ThemeProvider } from "styled-components";
import { styleReset, Button, Window, WindowHeader, WindowContent, ProgressBar, MenuList, MenuListItem } from "react95";
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

const Controls = styled.div`
  display: flex;
  justify-content: space-between;
  width: 100%;
  margin-top: 10px;
`;

const App: React.FC = () => {
  const [isPlaying, setIsPlaying] = useState(false);
  const [progress, setProgress] = useState(0);
  const [loading, setLoading] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const audioRef = useRef<HTMLAudioElement>(null);

  const handlePlayPause = () => {
    if (audioRef.current) {
      if (isPlaying) {
        audioRef.current.pause();
      } else {
        audioRef.current.play();
      }
      setIsPlaying(!isPlaying);
    }
  };

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      const file = e.target.files[0];
      const fileURL = URL.createObjectURL(file);
      setLoading(true);
      setProgress(0); // Reset progress bar
      console.log("Loading file:", file.name);

      try {
        const response = await fetch(fileURL);
        const reader = response.body?.getReader();
        const contentLength = +response.headers.get('Content-Length')!;
        
        let loaded = 0;
        reader?.read().then(function processResult(result) {
          if (result.done) {
            setLoading(false);
            setProgress(100);
            console.log("File loaded:", file.name);
            return;
          }

          loaded += result.value.length;
          setProgress((loaded / contentLength) * 100);

          return reader.read().then(processResult);
        });
        
        if (audioRef.current) {
          audioRef.current.src = fileURL;
          audioRef.current.load();
          setIsPlaying(false);
        }
      } catch (error) {
        console.error("Error loading file:", error);
        setLoading(false);
      }
    }
  };

  const handleMenuClick = () => {
    if (fileInputRef.current) {
      fileInputRef.current.click();
    }
  };

  return (
    <ThemeProvider theme={original}>
      <GlobalStyles />
      <Container>
        <Window style={{ width: 400 }}>
          <WindowHeader>
            <span>the-audio-player.exe</span>
          </WindowHeader>
          <WindowContent>
            <MenuList>
              <MenuListItem onClick={handleMenuClick}>
                <span role="img" aria-label="file">📁</span> Load File
              </MenuListItem>
            </MenuList>
            <input 
              type="file" 
              accept="audio/*" 
              ref={fileInputRef} 
              style={{ display: 'none' }} 
              onChange={handleFileChange} 
            />
            <audio ref={audioRef} />
            <Controls>
              <Button onClick={handlePlayPause}>
                {isPlaying ? 'Pause' : 'Play'}
              </Button>
              <Button onClick={() => {
                if (audioRef.current) {
                  audioRef.current.currentTime = 0;
                  setIsPlaying(false);
                  audioRef.current.pause();
                }
              }}>
                Stop
              </Button>
            </Controls>
            {loading ? (
              <ProgressBar value={progress} max={100} style={{ width: '100%', marginTop: '10px' }} />
            ) : null}
          </WindowContent>
        </Window>
      </Container>
    </ThemeProvider>
  );
};

export default App;