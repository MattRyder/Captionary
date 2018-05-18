import React from "react";
import PropTypes from 'prop-types'
import "./ImageContainer.css"

const FLICKR_KEY = process.env.REACT_APP_FLICKR_KEY;
const FLICKR_API = `https://api.flickr.com/services/rest/?method=flickr.interestingness.getList&api_key=${FLICKR_KEY}&format=rest&per_page=10&format=json&nojsoncallback=1`;

class Image {
	constructor(title, url) {
		this.title = title;
		this.url = url;
	}
}

/**
 * A component that contains a set of images.
 * It can download images via the Flickr API and render a single image at a time.
 */
class ImageContainer extends React.Component {

    constructor(props) {
		super(props)
		
		this.state = {
			images: [{}],
			imageIndex: props.imageIndex || 0,
			imageCentered: props.imageCentered || false
		}

		this.handleClick = this.handleClick.bind(this);
	}

	componentWillMount = () => {
	  this.loadImageSet();
	}

	handleClick() {
		this.setState({
			imageIndex: (this.state.imageIndex + 1) % this.state.images.length
		});
	}
	
	generateImageUrl(img) {
		var imgUrl = `https://farm${img.farm}.staticflickr.com/${img.server}/${img.id}_${img.secret}.jpg`;
		return new Image(img.title, imgUrl);
	}

	loadImageSet() {
		fetch(FLICKR_API)
		.then(res => res.json())
		.then(responseJson => {
			this.setState({
				images: responseJson.photos.photo.map(this.generateImageUrl)
			});
		});
	}

    render() {
		return (
			<div className={"image-container " + (this.state.imageCentered ? 'is-centered' : '')}>
				<img key={this.state.imageIndex}
					 src={this.state.images[this.state.imageIndex].url}
					 alt={this.state.images[this.state.imageIndex].title}
					 onClick={this.handleClick} />
			</div>
		)
    }    
};

ImageContainer.propTypes = {
	imageIndex: PropTypes.number,
	imageCentered: PropTypes.bool
}

export default ImageContainer;