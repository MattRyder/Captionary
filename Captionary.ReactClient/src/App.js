import React from 'react';
import { Switch, Route } from 'react-router-dom';
import { HubConnectionBuilder, LogLevel } from '@aspnet/signalr';
import Navigation from './components/Layout/Navigation';
import GameView from './components/Views/GameView/GameView';
import HomeView from './components/Views/HomeView/HomeView';

import './App.css';

const SERVER_HOST = process.env.REACT_APP_SIGNALR_HOST;

class App extends React.Component {

  constructor(props) {
    super(props);

    this.state = {
      hubConnection: null
    }
  }

  componentWillMount() {
    const hubConnection = new HubConnectionBuilder()
      .withUrl(SERVER_HOST)
      .configureLogging(LogLevel.Trace)
      .build();

    this.setState({ hubConnection }, () => {
        this.state.hubConnection
            .start()
            .catch(err => console.error("Failed to connect to Captionary."));
    })
  }

  render() {
    return (
        <div className="captionary-app">
           <Navigation />
           <Switch>
                <Route exact
                      path="/"
                      render={()=>{
                          return (<HomeView hubConnection={this.state.hubConnection}/>); }}/>
                <Route 
                      path="/game/:id"
                      render={()=>{
                          return (<GameView hubConnection={this.state.hubConnection}/>); }} />
            </Switch>
        </div>
    );
  }
}

export default App;
