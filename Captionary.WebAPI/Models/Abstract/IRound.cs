using System.Collections.Generic;
using Captionary.Models.Concrete;

namespace Captionary.Models.Abstract
{
    public interface IRound
    {
        List<Caption> PlayerCaptions { get; set; }

        string ID { get; }

        string ImageUrl { get; set; }

        ICaption WinningCaption { get; }
        
        bool SubmitCaption(Caption caption);

    };
}