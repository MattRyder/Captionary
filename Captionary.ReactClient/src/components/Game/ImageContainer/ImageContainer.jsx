import React from "react";
import { connect } from "react-redux";

import "./ImageContainer.css"

const mapStateToProps = (state) => {
	return { imageUrl: state.round.imageUrl };
}

class ImageContainerComponent extends React.Component {
	render() {
		return this.props.imageUrl ? (
			<div className={"image-container " + (this.props.imageCentered ? 'is-centered' : '')}>
				<img key={this.props}
					src={this.props.imageUrl}
					alt="ImageContainer" />
			</div>
		) : null;
	}
};

const ImageContainer = connect(mapStateToProps, null)(ImageContainerComponent);
export default ImageContainer;