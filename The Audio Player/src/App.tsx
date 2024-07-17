import React, { useState, useRef } from "react";
import styled, { createGlobalStyle, ThemeProvider } from "styled-components";
import { styleReset, Button, Window, WindowHeader, WindowContent, Slider, ProgressBar, MenuList, MenuListItem, Monitor, AppBar, Toolbar } from "react95";
import original from "react95/dist/themes/millenium"; // Use original theme for GUI components
import "./App.css";
import { invoke } from '@tauri-apps/api/tauri';

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

const App: React.FC = () => {
  const [isPlaying, setIsPlaying] = useState(false);
  const [progress, setProgress] = useState(0);
  const [loading, setLoading] = useState(false);
  const [songName, setSongName] = useState("");
  const [volume, setVolume] = useState(50); // Added state for slider (volume control)
  const fileInputRef = useRef<HTMLInputElement>(null);
  const audioRef = useRef<HTMLAudioElement>(null);
  const timerRef = useRef<NodeJS.Timeout | null>(null);

  const handlePlayPause = () => {
    if (isPlaying) {
      invoke('pause');
    } else {
      invoke('play');
    }
    setIsPlaying(!isPlaying);
  };

  const handleVolumeChange = (value: number) => {
    setVolume(value);
    if (audioRef.current) {
      audioRef.current.volume = value / 100; // Volume range is 0 to 1
    }
  };

  const simulateLoadingProgress = (file: File) => {
    const reader = new FileReader();
    
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
        setSongName(file.name); // Set the song name from the file name
        invoke('play', { path: reader.result as string });
      }, 500); // Delay hiding the progress bar to show 100% for a moment
    };

    reader.onerror = () => {
      console.error("Error loading file:", reader.error);
      setLoading(false);
    };

    reader.readAsDataURL(file);
    startProgressSimulation(); // Start the fake progress simulation
  };

  const startProgressSimulation = () => {
    if (timerRef.current) {
      clearInterval(timerRef.current);
    }
    timerRef.current = setInterval(() => {
      setProgress(prev => {
        const nextProgress = prev + 1;
        if (nextProgress >= 100) {
          clearInterval(timerRef.current!);
          return 100;
        }
        return nextProgress;
      });
    }, 100); // Update progress every 100ms
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      const file = e.target.files[0];
      if (file.name.length <= 255) {
        simulateLoadingProgress(file);
      } else {
        console.error("File path is too long");
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
                <Monitor
                  backgroundStyles={{ background: 'white' }}
                  textStyles={{ color: 'lime' }} // Change text color to lime (green)
                  showSideButtons={false}
                  style={{ marginLeft: '10px', width: '200px' }}
                >
                  <div>{songName || "No song loaded"}</div>
                </Monitor>
              </MonitorContainer>
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
                  invoke('stop');
                  setIsPlaying(false);
                }}>
                  Stop
                </Button>
                <Slider
                  min={0}
                  max={100}
                  value={volume}
                  onChange={handleVolumeChange}
                  width={150}
                  style={{ marginLeft: '10px' }}
                />
              </Controls>
              {loading && (
                <div style={{ width: '100%', marginTop: '10px' }}>
                  <ProgressBar value={progress} />
                </div>
              )}
            </WindowContent>
          </Window>
        </WindowContainer>
      </AppContainer>
    </ThemeProvider>
  );
};

export default App;