import React from 'react';
import { Switch, Route } from 'react-router-dom';

import Navigation from './components/Layout/Navigation';
import GameView from './components/Views/GameView/GameView';
import HomeView from './components/Views/HomeView/HomeView';
import ErrorView from './components/Views/ErrorView/ErrorView';

import './App.css';

class App extends React.Component {

  constructor(props) {
    super(props);

    this.state = {
      hubConnection: null
    }
  }

  render() {
    return (
      <div className="captionary-app">
        <Navigation />
        <Switch>
          <Route exact
            path="/"
            render={() => {
              return (<HomeView />);
            }} />
          <Route
            path="/game/:id"
            render={() => {
              return (<GameView />);
            }} />
          <Route
            path="/error/:id"
            component={ErrorView} />
        </Switch>
      </div>
    );
  }
}

export default App;
