using System;
using System.Collections.Generic;
using System.Linq;
using Captionary.Models.Abstract;
using ProtoBuf;

namespace Captionary.Models.Concrete
{
    [ProtoContract]
    public class Round : IRound
    {
        string id;
        string imageUrl;
        Dictionary<IPlayer, ICaption> playerCaptions;

        public Round()
        {
            this.id = Guid.NewGuid().ToString();
            playerCaptions = new Dictionary<IPlayer, ICaption>();
        }

        [ProtoMember(1)]
        public string ID
        {
            get { return id; }
            set { this.id = value; }
        }

        [ProtoMember(2)]
        public string ImageUrl
        {
            get { return imageUrl; }
            set { imageUrl = value; }
        }

        public Dictionary<IPlayer, ICaption> PlayerCaptions
        {
            get { return playerCaptions; }
            set { playerCaptions = value; }
        }

        public IPlayer WinningPlayer
        {
            get
            {
                return playerCaptions
                    .OrderByDescending(pcSet => pcSet.Value.Points)
                    .First().Key;
            }
        }

        public bool SubmitCaption(IPlayer player, ICaption caption)
        {
            if (playerCaptions.ContainsKey(player))
            {
                return false;
            }

            playerCaptions.Add(player, caption);
            return true;
        }
    }
}