namespace Captionary.Models.Abstract
{
    public interface ICaption
    {
        string ID { get; set; }

        string PlayerID { get; set; }

        string Text { get; set; } 

        int Points { get; set; }

        void AddPoints(int points);
    }
}