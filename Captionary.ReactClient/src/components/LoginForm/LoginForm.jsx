import React from 'react';
import { Button } from 'reactstrap';
import { connect } from 'react-redux';

import { LoginAction } from '../../actions/LoginAction';

import Input from '../Input/Input';
import './LoginForm.css';

const mapDispatchToProps = (dispatch) => {
  return {
    LoginAction: (playerName) => dispatch(LoginAction(playerName, null))
  };
};

class LoginFormComponent extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      playerName: "",
    }

    this.handleNameChange = this.handleNameChange.bind(this);
    this.onSubmit = this.onSubmit.bind(this);
  }

  handleNameChange(e) {
    this.setState({ playerName: e.target.value });
  }

  onSubmit(e) {
    e.preventDefault();
    this.props.LoginAction(this.state.playerName);
  }

  render() {
    return (
      <div className="login-form">
        <Input onChange={this.handleNameChange}
          value={this.state.playerName}
          placeholder="What is your name?" />

        <Button color='success' block onClick={this.onSubmit}>
          Play
        </Button>
      </div>
    )
  }
};

const LoginForm = connect(null, mapDispatchToProps)(LoginFormComponent);
export default LoginForm;