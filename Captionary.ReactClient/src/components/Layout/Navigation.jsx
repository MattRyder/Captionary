import React from 'react';
import {
  Collapse,
  Navbar,
  NavbarToggler,
  NavbarBrand,
  Nav,
  NavItem,
  NavLink } from 'reactstrap';

import "./Navigation.css";

export default class Navigation extends React.Component {
  constructor(props) {
    super(props);

    this.toggle = this.toggle.bind(this);

    this.state = {
      isOpen: false
    }    
  }

  toggle() {
    this.setState({ isOpen: !this.state.isOpen });
  }

  render() {
    return (
      <Navbar color="dark" expand="sm">
        <NavbarBrand href="/">Captionary</NavbarBrand>
        <NavbarToggler onClick={this.toggle} />
        <Collapse isOpen={this.state.isOpen} navbar />
          <Nav>
            <NavItem>
              <NavLink href="/game">Game</NavLink>
            </NavItem>
          </Nav>
      </Navbar>
    );
  }

}