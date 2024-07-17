import React, { useState, useRef } from "react";
import styled, { createGlobalStyle, ThemeProvider } from "styled-components";
import { styleReset, Button, Window, WindowHeader, WindowContent, Slider, MenuList, MenuListItem, Monitor, AppBar, Toolbar } from "react95";
import original from "react95/dist/themes/original"; 
import { invoke } from '@tauri-apps/api/tauri';
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
  height: 30px; /* Make the AppBar thinner */
`;

const StyledMonitor = styled(Monitor)`
  margin-left: 10px;
  width: 200px;
  background: white;
  color: lime;
`;

const App: React.FC = () => {
  const [songName, setSongName] = useState("");
  const [volume, setVolume] = useState(50); 
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handlePlayPause = async () => {
    if (fileInputRef.current && fileInputRef.current.files && fileInputRef.current.files.length > 0) {
      const file = fileInputRef.current.files[0];
      const reader = new FileReader();
      reader.onload = () => {
        const fileInfo = {
          path: reader.result as string
        };

        try {
          invoke('play', { fileInfo });
        } catch (error) {
          console.error('Error:', error);
        }
      };
      reader.readAsDataURL(file);
    }
  };

  const handleVolumeChange = (value: number) => {
    setVolume(value);
   
  };

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      const file = e.target.files[0];
      setSongName(file.name); 
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
                style={{ height: '45px',marginRight: '4px' }}
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
                <StyledMonitor>
                  <div>{songName || "No song loaded"}</div>
                </StyledMonitor>
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
                  Play/Pause
                </Button>
                <Button onClick={() => {
                 
                }}>
                  Stop
                </Button>
                <Slider
                  min={0}
                  max={100}
                  value={volume}
                  onChange={handleVolumeChange}
                />
              </Controls>
            </WindowContent>
          </Window>
        </WindowContainer>
      </AppContainer>
    </ThemeProvider>
  );
};

export default App;