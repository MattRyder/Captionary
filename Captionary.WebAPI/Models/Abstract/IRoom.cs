using System;
using System.Collections.Generic;

namespace Captionary.Models.Abstract
{
    public interface IRoom
    {
        string ID { get; set; }

        HashSet<IPlayer> Players { get; set; }

        LinkedList<IRound> Rounds { get; set; }

        bool AddPlayer(IPlayer player);

        bool RemovePlayer(IPlayer player);

        bool AddRound(IRound round);


    }
}