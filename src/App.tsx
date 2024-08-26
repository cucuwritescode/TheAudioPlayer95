import React, { useState, useRef, useEffect } from "react";
import styled, { createGlobalStyle, ThemeProvider } from "styled-components";
import { styleReset, Button, Window, WindowHeader, WindowContent, Slider, MenuList, MenuListItem, AppBar, Toolbar, Monitor } from "react95";
import original from "react95/dist/themes/millenium";
import "./App.css";
import { invoke } from "@tauri-apps/api/tauri";

// Global styles for the application
const GlobalStyles = createGlobalStyle`
  ${styleReset}
  body {
    font-family: 'ms_sans_serif';
    background-image: url('https://baloo.neocities.org/Images/planetbg5.jpg');
    background-size: cover;
    background-position: center;
  }
`;

// Styled components
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

const FolderList = styled.ul`
  list-style: none;
  padding-left: 10px;
  font-family: "Courier New", Courier, monospace;
`;

const FolderItem = styled.li`
  margin: 5px 0;
  cursor: pointer;
  font-size: 14px; /* Adjusted font size */
  &:hover {
    text-decoration: underline;
  }
`;

const StyledAppBar = styled(AppBar)`
  width: 100%;
  height: 30px;
`;

// TypeScript type for audio tracks
interface AudioTrack {
  id: string;
  name: string;
  path: string;
}

// Main component
const App: React.FC = () => {
  const [isPlaying, setIsPlaying] = useState(false);
  const [songName, setSongName] = useState("");
  const [volume, setVolume] = useState(0.5);
  const audioRef = useRef<HTMLAudioElement | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [tracks, setTracks] = useState<AudioTrack[]>([]);

  // Load tracks on component mount
  useEffect(() => {
    async function fetchTracks() {
      const tracks = await listTracks();
      setTracks(tracks);
    }
    fetchTracks();
  }, []);

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
    setVolume(value);
    if (audioRef.current) {
      audioRef.current.volume = value;
    }
  };

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      const file = e.target.files[0];
      const fileURL = URL.createObjectURL(file);
      setSongName(file.name);
      await addTrack(file.name, file.name, fileURL);
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

  const handleTrackClick = (track: AudioTrack) => {
    setSongName(track.name);
    if (audioRef.current) {
      audioRef.current.src = track.path;
      audioRef.current.play();
      setIsPlaying(true);
    }
  };

  // Functions for CRUD operations
  async function addTrack(id: string, name: string, path: string) {
    await invoke("add_audio_track", { id, name, path });
    setTracks((prevTracks) => [...prevTracks, { id, name, path }]);
  }

  async function removeTrack(id: string) {
    await invoke("remove_audio_track", { id });
    setTracks((prevTracks) => prevTracks.filter((track) => track.id !== id));
  }

  async function listTracks(): Promise<AudioTrack[]> {
    return await invoke("list_audio_tracks");
  }

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
          <Window style={{ width: 750 }}>
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
                <Monitor background="white" text="black" style={{ width: '400px', height: '100px', marginLeft: '15px' }}>
                  <div style={{ padding: '5px', color: '#000000' }}>
                    {songName || "No song loaded"}
                  </div>
                </Monitor>
                <FolderList>
                  {tracks.map((track) => (
                    <FolderItem key={track.id} onClick={() => handleTrackClick(track)}>
                      {track.name}
                    </FolderItem>
                  ))}
                </FolderList>
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
                  step={0.001}
                  value={volume}
                  onChange={(e) => handleVolumeChange(Number(e))}
                  style={{ marginLeft: '330px', width: '300px' }} // Adjust the width here
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