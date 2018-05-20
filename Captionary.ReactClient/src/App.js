import React from 'react';
import './App.css';

import Navigation from './components/Layout/Navigation';
import ContentRouter from './components/Layout/ContentRouter';

class App extends React.Component {
  render() {
    return (
        <div className="captionary-app">
           <Navigation />
           <ContentRouter />
        </div>
    );
  }
}

export default App;
