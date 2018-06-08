using System;
using System.Collections.Generic;
using System.Linq;
using Captionary.Models.Abstract;

namespace Captionary.Models.Concrete
{
    public class Round : IRound
    {
        string id;
        string imageUrl;
        List<Caption> playerCaptions;

        public Round()
        {
            this.id = Guid.NewGuid().ToString();
            playerCaptions = new List<Caption>();
        }

        public string ID
        {
            get { return id; }
            set { this.id = value; }
        }

        public string ImageUrl
        {
            get { return imageUrl; }
            set { imageUrl = value; }
        }

        public List<Caption> PlayerCaptions
        {
            get { return playerCaptions; }
            set { playerCaptions = value; }
        }

        public ICaption WinningCaption
        {
            get
            {
                return playerCaptions
                    .OrderByDescending(caps => caps.Points)
                    .FirstOrDefault();
            }
        }

        public bool SubmitCaption(Caption caption)
        {
            var playerHasSubmitted = playerCaptions.Any(caps => caps.PlayerID == caption.PlayerID);
            if (playerHasSubmitted || string.IsNullOrEmpty(caption.PlayerID))
            {
                return false;
            }

            playerCaptions.Add(caption);
            return true;
        }
    }
}