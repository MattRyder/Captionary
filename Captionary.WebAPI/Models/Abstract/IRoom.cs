using System;
using System.Collections.Generic;
using Captionary.Models.Concrete;

namespace Captionary.Models.Abstract
{
    public interface IRoom
    {
        string ID { get; set; }

        HashSet<Player> Players { get; set; }

        LinkedList<Round> Rounds { get; set; }

        bool AddPlayer(Player player);

        bool RemovePlayer(Player player);

        bool AddRound(Round round);


    }
}