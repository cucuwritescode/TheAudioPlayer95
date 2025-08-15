import React, { useState, useRef, useEffect } from "react";
import styled, { createGlobalStyle, ThemeProvider } from "styled-components";
import { styleReset, Button, Window, WindowHeader, WindowContent, Slider, MenuList, MenuListItem, AppBar, Toolbar, Monitor, GroupBox } from "react95";
import original from "react95/dist/themes/millenium";
import "./App.css";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { TreeView } from "react95";

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


const CustomGroupBox = styled(GroupBox)`
  margin-left: 5px;  
  margin-right: -300px;  
  padding: 5px;
  width: 230px;  
  height: 230px; 
  overflow: auto;
`;


const CustomTreeView = styled(TreeView)`
  & .tree-node {
    font-size: 12px; /*  */
    max-width: 200px; /*  */
    overflow: hidden; /*  */
    text-overflow: ellipsis; /*  */
    white-space: nowrap; /*  */
  }

  & .tree-node:hover {
    overflow: visible;
    white-space: normal; /*  */
  }

  & .tree-label {
    max-width: 200px; /*  */
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
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
  const [tracks, setTracks] = useState<AudioTrack[]>([]);

  // Load tracks on component mount
  useEffect(() => {
    async function fetchTracks() {
      const tracks = await listTracks();
      setTracks(tracks);
    }
    fetchTracks();
  }, []);

  const handlePlayPause = async () => {
    try {
      if (isPlaying) {
        await invoke("pause_audio");
      } else {
        await invoke("play_audio");
      }
      setIsPlaying(!isPlaying);
    } catch (error) {
      console.error("Error toggling playback:", error);
    }
  };

  const handleVolumeChange = async (value: number) => {
    setVolume(value);
    try {
      // Convert 0-1 range to 0-100 for Rust backend
      const volumePercent = Math.round(value * 100);
      await invoke("set_volume", { volume: volumePercent });
    } catch (error) {
      console.error("Error setting volume:", error);
    }
  };


  const handleMenuClick = async () => {
    try {
      // Use Tauri's file dialog
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Audio',
          extensions: ['mp3', 'wav', 'flac', 'aac', 'ogg', 'm4a']
        }]
      });
      
      if (selected && typeof selected === 'string') {
        const fileName = selected.split('/').pop() || selected.split('\\').pop() || 'Unknown';
        setSongName(fileName);
        
        // Add to tracks and play
        await addTrack(fileName, fileName, selected);
        await invoke("play_track", { filePath: selected });
        setIsPlaying(true);
      }
    } catch (error) {
      console.error("Error selecting file:", error);
    }
  };

  const handleTrackClick = async (track: AudioTrack) => {
    setSongName(track.name);
    try {
      await invoke("play_track", { filePath: track.path });
      setIsPlaying(true);
    } catch (error) {
      console.error("Error playing track:", error);
    }
  };

  // Prepare tree data
  const data = [
    {
      id: 'root',
      label: 'Music Library',
      items: tracks.map(track => ({
        id: track.id,
        label: track.name,
      })),
    },
  ];

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
                  <div style={{ padding: '10px', color: '#000000' }}>
                    {songName || "No songs loaded"}
                  </div>
                </Monitor>
                <CustomGroupBox label="" style={{ width: '250px', 
    height: '200px', 
    overflowY: 'auto', 
    marginRight: '10px', // Adjust margin as needed
    padding: '10px' }}>
  <CustomTreeView
  tree={data}
  onNodeSelect={async (_, id) => {
    const selectedTrack = tracks.find(track => track.id === id);
    if (selectedTrack) {
      setSongName(selectedTrack.name); // Update song name in the monitor
      try {
        await invoke("play_track", { filePath: selectedTrack.path });
        setIsPlaying(true);
      } catch (error) {
        console.error("Error playing track:", error);
      }
    }
  }}
  style={{
    fontSize: '12px',
    maxWidth: '200px',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    whiteSpace: 'nowrap',
  }}
/>
</CustomGroupBox>
              </MonitorContainer>
              <Controls>
                <Button onClick={handlePlayPause}>
                  {isPlaying ? 'Pause' : 'Play'}
                </Button>
                <Button onClick={async () => {
                  try {
                    await invoke("stop_audio");
                    setIsPlaying(false);
                  } catch (error) {
                    console.error("Error stopping audio:", error);
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
      </AppContainer>
    </ThemeProvider>
  );
};

export default App;