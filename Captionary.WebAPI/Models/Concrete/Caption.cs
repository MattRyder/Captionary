using Captionary.Models.Abstract;

namespace Captionary.Models.Concrete
{
    public class Caption : ICaption
    {
        string id;
        string text;
        int points;

        public string ID
        {
            get { return id; }
            set { id = value; }
        }

        public string Text
        {
            get { return text; }
            set { text = value; }
        }

        public int Points
        {
            get { return points; }
            set { points = value; }
        }

        public void AddPoints(int points)
        {
            this.points += points;
        }
    }
}