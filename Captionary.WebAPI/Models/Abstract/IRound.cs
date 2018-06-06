using System.Collections.Generic;

namespace Captionary.Models.Abstract
{
    public interface IRound
    {
        Dictionary<IPlayer, ICaption> PlayerCaptions { get; set; }

        string ID { get; }

        string ImageUrl { get; set; }

        IPlayer WinningPlayer { get; }
        
        bool SubmitCaption(IPlayer player, ICaption caption);

    };
}