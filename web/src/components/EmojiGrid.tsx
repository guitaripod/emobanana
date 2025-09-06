import { useState, useCallback } from 'react';

const FACIAL_EMOJIS = [
  '😀', '😃', '😄', '😁', '😆', '😅', '😂', '🤣', '😊', '😇',
  '🙂', '🙃', '😉', '😌', '😍', '🥰', '😘', '😗', '😙', '😚',
  '😋', '😛', '😝', '😜', '🤪', '🤨', '🧐', '🤓', '😎', '🤩',
  '🥳', '😏', '😒', '😞', '😔', '😟', '😕', '🙁', '☹️', '😣',
  '😖', '😫', '😩', '🥺', '😢', '😭', '😤', '😠', '😡', '🤬',
  '🤯', '😳', '🥵', '🥶', '😱', '😨', '😰', '😥', '😓', '🤗',
  '🤔', '🤭', '🤫', '🤥', '😶', '😐', '😑', '😬', '🙄', '😯',
  '😦', '😧', '😮', '😲', '🥱', '😴', '🤤', '😪', '😵', '🤐',
  '🥴', '🤢', '🤮', '🤧', '😷', '🤒', '🤕', '🤑', '🤠'
];

interface EmojiGridProps {
  onEmojiSelect: (emoji: string) => void;
  selectedEmoji: string | null;
}

export default function EmojiGrid({ onEmojiSelect, selectedEmoji }: EmojiGridProps) {
  const [hoveredEmoji, setHoveredEmoji] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [recentEmojis, setRecentEmojis] = useState<string[]>(() => {
    if (typeof window !== 'undefined') {
      const saved = localStorage.getItem('recentEmojis');
      return saved ? JSON.parse(saved) : [];
    }
    return [];
  });

  const handleEmojiClick = useCallback((emoji: string) => {
    onEmojiSelect(emoji);
    
    const newRecent = [emoji, ...recentEmojis.filter(e => e !== emoji)].slice(0, 8);
    setRecentEmojis(newRecent);
    if (typeof window !== 'undefined') {
      localStorage.setItem('recentEmojis', JSON.stringify(newRecent));
    }
  }, [onEmojiSelect, recentEmojis]);

  const getEmojiName = (emoji: string) => {
    const names: Record<string, string> = {
      '😀': 'Grinning', '😃': 'Happy', '😄': 'Smile', '😁': 'Beaming',
      '😆': 'Laughing', '😅': 'Sweat Smile', '😂': 'Joy', '🤣': 'Rolling',
      '😊': 'Blush', '😇': 'Innocent', '🙂': 'Slight Smile', '🙃': 'Upside Down',
      '😉': 'Wink', '😌': 'Relieved', '😍': 'Heart Eyes', '🥰': 'Smiling Hearts',
      '😘': 'Kiss', '😗': 'Kissing', '😙': 'Kiss Smile', '😚': 'Kiss Closed',
      '😋': 'Yum', '😛': 'Tongue', '😝': 'Tongue Wink', '😜': 'Crazy',
      '🤪': 'Zany', '🤨': 'Raised Eyebrow', '🧐': 'Monocle', '🤓': 'Nerd',
      '😎': 'Cool', '🤩': 'Star Eyes', '🥳': 'Party', '😏': 'Smirk',
      '😒': 'Unamused', '😞': 'Disappointed', '😔': 'Pensive', '😟': 'Worried',
      '😕': 'Confused', '🙁': 'Frown', '☹️': 'Sad', '😣': 'Persevere',
      '😖': 'Confounded', '😫': 'Tired', '😩': 'Weary', '🥺': 'Pleading',
      '😢': 'Cry', '😭': 'Sob', '😤': 'Triumph', '😠': 'Angry',
      '😡': 'Rage', '🤬': 'Cursing', '🤯': 'Exploding', '😳': 'Flushed',
      '🥵': 'Hot', '🥶': 'Cold', '😱': 'Scream', '😨': 'Fearful',
      '😰': 'Anxious', '😥': 'Sad Sweat', '😓': 'Sweat', '🤗': 'Hug',
      '🤔': 'Thinking', '🤭': 'Hand Mouth', '🤫': 'Shush', '🤥': 'Lying',
      '😶': 'No Mouth', '😐': 'Neutral', '😑': 'Expressionless', '😬': 'Grimace',
      '🙄': 'Eye Roll', '😯': 'Hushed', '😦': 'Frowning', '😧': 'Anguished',
      '😮': 'Open Mouth', '😲': 'Astonished', '🥱': 'Yawn', '😴': 'Sleep',
      '🤤': 'Drool', '😪': 'Sleepy', '😵': 'Dizzy', '🤐': 'Zipper',
      '🥴': 'Woozy', '🤢': 'Nauseated', '🤮': 'Vomit', '🤧': 'Sneeze',
      '😷': 'Mask', '🤒': 'Thermometer', '🤕': 'Bandage', '🤑': 'Money',
      '🤠': 'Cowboy'
    };
    return names[emoji] || 'Emoji';
  };

  const filteredEmojis = searchQuery
    ? FACIAL_EMOJIS.filter(emoji => 
        getEmojiName(emoji).toLowerCase().includes(searchQuery.toLowerCase())
      )
    : FACIAL_EMOJIS;

  return (
    <div className="w-full max-w-4xl mx-auto">
      <div className="mb-6">
        <input
          type="text"
          placeholder="Search emojis..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="w-full px-4 py-3 rounded-xl glass border-0 focus:outline-none focus:ring-2 focus:ring-orange-400 text-lg dark:text-slate-100"
        />
      </div>

      {recentEmojis.length > 0 && !searchQuery && (
        <div className="mb-6">
          <p className="text-sm font-medium text-slate-600 dark:text-slate-400 mb-3">Recently Used</p>
          <div className="flex gap-2 flex-wrap">
            {recentEmojis.map((emoji) => (
              <button
                key={`recent-${emoji}`}
                onClick={() => handleEmojiClick(emoji)}
                className={`
                  text-3xl p-3 rounded-xl glass emoji-button
                  ${selectedEmoji === emoji ? 'ring-2 ring-orange-400 bg-orange-50 dark:bg-orange-950/30' : ''}
                `}
                aria-label={`Select ${getEmojiName(emoji)}`}
              >
                {emoji}
              </button>
            ))}
          </div>
        </div>
      )}

      <div className="glass rounded-2xl p-6">
        <div className="grid grid-cols-5 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 gap-2">
          {filteredEmojis.map((emoji) => (
            <button
              key={emoji}
              onClick={() => handleEmojiClick(emoji)}
              onMouseEnter={() => setHoveredEmoji(emoji)}
              onMouseLeave={() => setHoveredEmoji(null)}
              className={`
                relative text-3xl lg:text-4xl p-3 rounded-xl emoji-button
                hover:bg-white/50 dark:hover:bg-slate-700/50 active:bg-white/70 dark:active:bg-slate-700/70
                ${selectedEmoji === emoji ? 'ring-2 ring-orange-400 bg-orange-50 dark:bg-orange-950/30' : ''}
                focus:outline-none focus:ring-2 focus:ring-orange-400
              `}
              aria-label={`Select ${getEmojiName(emoji)}`}
            >
              {emoji}
              {hoveredEmoji === emoji && (
                <span className="absolute -top-8 left-1/2 transform -translate-x-1/2 bg-slate-800 dark:bg-slate-700 text-white text-xs py-1 px-2 rounded whitespace-nowrap z-10">
                  {getEmojiName(emoji)}
                </span>
              )}
            </button>
          ))}
        </div>
        
        {filteredEmojis.length === 0 && (
          <p className="text-center text-slate-500 dark:text-slate-400 py-8">No emojis found matching "{searchQuery}"</p>
        )}
      </div>
    </div>
  );
}