import React from "react";

import "./ImageContainer.css"

export default class ImageContainerComponent extends React.Component {
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