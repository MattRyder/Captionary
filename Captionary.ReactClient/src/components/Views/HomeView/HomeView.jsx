import React from 'react';
import LoginForm from '../../LoginForm/LoginForm';

import "./HomeView.css";

export default class HomeView extends React.Component {
    constructor(props) {
        super(props);

        this.state = { };
    }

    render() {
        return (
            <div className="home-view">
                <LoginForm />
            </div>
        );
    }
}