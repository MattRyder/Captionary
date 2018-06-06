import React from "react";
import PropTypes from 'prop-types'

import "./ImageContainer.css"
import { connect } from "react-redux";

const mapStateToProps = (state) => {
	return { imageUrl: state.round.imageUrl };
}

class _ImageContainer extends React.Component {

	render() {
		return (
			<div className={"image-container " + (this.props.imageCentered ? 'is-centered' : '')}>
				<img key={this.props}
					src={this.props.imageUrl}
					alt="ImageContainer" />
			</div>
		)
	}
};

const ImageContainer = connect(mapStateToProps, null)(_ImageContainer);
export default ImageContainer;