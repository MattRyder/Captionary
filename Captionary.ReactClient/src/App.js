import React, { Component } from 'react';
import './App.css';

import ImageContainer from "./components/ImageContainer/ImageContainer.js";
import CaptionInput from "./components/CaptionInput/CaptionInput.js"
import CaptionCardList from "./components/CaptionCardList/CaptionCardList.js"
import ChatContainer from './components/ChatContainer/ChatContainer.js'

class App extends Component {
  render() {
    return (
        <div className="app">
            <div className="app-game">
                <CaptionCardList/>
                <ImageContainer imageCentered={true} />
                <CaptionInput />
            </div>
            <div className="app-chat">
                <ChatContainer name={"Matt"} />
            </div>
        </div>
    );
  }
}

export default App;
