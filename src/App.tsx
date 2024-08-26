import React, { useState, useRef } from "react";
import styled, { createGlobalStyle, ThemeProvider } from "styled-components";
import { styleReset, Button, Window, WindowHeader, WindowContent, Slider, MenuList, MenuListItem, AppBar, Toolbar } from "react95";
import original from "react95/dist/themes/millenium";
import "./App.css";

const GlobalStyles = createGlobalStyle`
  ${styleReset}
  body {
    font-family: 'ms_sans_serif';
    background-image: url('https://baloo.neocities.org/Images/planetbg5.jpg');
    background-size: cover;
    background-position: center;
  }
`;

const AppContainer = styled.div`
  height: 100vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  position: relative;
`;

const WindowContainer = styled.div`
  flex-grow: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
`;

const Controls = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  margin-top: 10px;
`;

const MonitorContainer = styled.div`
  display: flex;
  align-items: center;
`;

const StyledAppBar = styled(AppBar)`
  width: 100%;
  height: 30px;
`;

const App: React.FC = () => {
  const [isPlaying, setIsPlaying] = useState(false);
  const [songName, setSongName] = useState("");
  const [volume, setVolume] = useState(0.5); //rango de volumen de 0.0 a 1.0
  const audioRef = useRef<HTMLAudioElement | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

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

  const handleVolumeChange = (value: number) => {
    console.log("Volume changed:", value);
    setVolume(value);
    if (audioRef.current) {
      audioRef.current.volume = value;
    }
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      const file = e.target.files[0];
      const fileURL = URL.createObjectURL(file);
      setSongName(file.name);
      if (audioRef.current) {
        audioRef.current.src = fileURL;
        audioRef.current.play();
        setIsPlaying(true);
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
      <AppContainer>
        <StyledAppBar>
          <Toolbar>
            <Button variant="menu" size="sm">
              <img
                src="https://64.media.tumblr.com/33e368bd4b99ee756fb59d367972e0b4/a3308f90a5978617-32/s540x810/970a4ede9f82f9db2069ce999a2754e7ee98e29a.png"
                alt="start logo"
                style={{ height: '45px', marginRight: '4px' }}
              />
              The Audio Player95
            </Button>
          </Toolbar>
        </StyledAppBar>
        <WindowContainer>
          <Window style={{ width: 600 }}>
            <WindowHeader>
              <span>the-audio-player95.exe</span>
            </WindowHeader>
            <WindowContent>
              <MonitorContainer>
                <MenuList>
                  <MenuListItem onClick={handleMenuClick}>
                    <span role="img" aria-label="file">📁</span> Load File
                  </MenuListItem>
                </MenuList>
                <div style={{ background: '#ffffff', color: '#00ff00', marginLeft: '10px', width: '200px' }}>
                  <div>{songName || "No song loaded"}</div>
                </div>
              </MonitorContainer>
              <input 
                type="file" 
                accept="audio/*" 
                ref={fileInputRef} 
                style={{ display: 'none' }} 
                onChange={handleFileChange} 
              />
              <Controls>
                <Button onClick={handlePlayPause}>
                  {isPlaying ? 'Pause' : 'Play'}
                </Button>
                <Button onClick={() => {
                  if (audioRef.current) {
                    audioRef.current.pause();
                    audioRef.current.currentTime = 0;
                    setIsPlaying(false);
                  }
                }}>
                  Stop
                </Button>
                <Slider
                  min={0}
                  max={1}
                  step={0.01}
                  value={volume}
                  onChange={(value: number) => handleVolumeChange(value)}
                  style={{ marginLeft: '10px', width: '150px' }}
                />
              </Controls>
            </WindowContent>
          </Window>
        </WindowContainer>
        <audio ref={audioRef} />
      </AppContainer>
    </ThemeProvider>
  );
};

export default App;