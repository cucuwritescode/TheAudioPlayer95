import React, { useState, useRef, useEffect } from "react";
import styled, { createGlobalStyle, ThemeProvider } from "styled-components";
import { styleReset, Button, Window, WindowHeader, WindowContent, ProgressBar, MenuList, MenuListItem } from "react95";
import original from "react95/dist/themes/original"; // Use original theme for GUI components
import "./App.css";

const GlobalStyles = createGlobalStyle`
  ${styleReset}
  body {
    font-family: 'ms_sans_serif';
    background-image: url('https://baloo.neocities.org/Images/planetbg5.jpg'); // Use the image URL you found
    background-size: cover;
    background-position: center;
  }
`;

const AppContainer = styled.div`
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
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

  const simulateLoadingProgress = (file: File) => {
    const reader = new FileReader();
    const fakeProgressIncrement = 1; // Increment progress by 1% each step
    const fakeLoadingTime = 500; // 500ms per step to make the progress bar slower

    reader.onloadstart = () => {
      console.log("File loading started");
      setLoading(true);
      setProgress(0); // Reset progress bar
    };

    reader.onloadend = () => {
      console.log("File loading ended");
      setProgress(100);
      setTimeout(() => {
        setLoading(false);
        if (audioRef.current) {
          audioRef.current.src = reader.result as string;
          audioRef.current.load();
          setIsPlaying(false);
        }
      }, 500); // Delay hiding the progress bar to show 100% for a moment
    };

    reader.onerror = () => {
      console.error("Error loading file:", reader.error);
      setLoading(false);
    };

    const simulateProgress = () => {
      setTimeout(() => {
        setProgress((prev) => {
          const nextProgress = prev + fakeProgressIncrement;
          console.log(`Updating progress: ${nextProgress}%`);
          if (nextProgress >= 100) {
            reader.onloadend(null as any);
            return 100;
          } else {
            simulateProgress();
            return nextProgress;
          }
        });
      }, fakeLoadingTime);
    };

    console.log("Starting to read file");
    reader.readAsDataURL(file);
    simulateProgress();
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      const file = e.target.files[0];
      simulateLoadingProgress(file);
    }
  };

  const handleMenuClick = () => {
    if (fileInputRef.current) {
      fileInputRef.current.click();
    }
  };

  useEffect(() => {
    console.log(`Loading: ${loading}, Progress: ${progress}%`);
  }, [loading, progress]);

  return (
    <ThemeProvider theme={original}>
      <GlobalStyles />
      <AppContainer>
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
            {loading && (
              <div style={{ width: '100%', marginTop: '10px' }}>
                <ProgressBar value={progress} />
              </div>
            )}
          </WindowContent>
        </Window>
      </AppContainer>
    </ThemeProvider>
  );
};

export default App;