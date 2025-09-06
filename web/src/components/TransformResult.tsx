import { useState } from 'react';

interface TransformResultProps {
  originalImage: string;
  transformedImage: string;
  processingTime?: number;
  onReset: () => void;
}

export default function TransformResult({ 
  originalImage, 
  transformedImage, 
  processingTime,
  onReset 
}: TransformResultProps) {
  const [viewMode, setViewMode] = useState<'side-by-side' | 'slider'>('side-by-side');
  const [sliderPosition, setSliderPosition] = useState(50);

  const handleDownload = () => {
    const link = document.createElement('a');
    link.href = `data:image/jpeg;base64,${transformedImage}`;
    link.download = `emobanana-${Date.now()}.jpg`;
    link.click();
  };

  return (
    <div className="w-full max-w-6xl mx-auto space-y-6">
      <div className="flex justify-center gap-2">
        <button
          onClick={() => setViewMode('side-by-side')}
          className={`px-4 py-2 rounded-lg transition-all ${
            viewMode === 'side-by-side' 
              ? 'bg-orange-400 text-white' 
              : 'glass hover:bg-white/50 dark:hover:bg-slate-700/50'
          }`}
        >
          Side by Side
        </button>
        <button
          onClick={() => setViewMode('slider')}
          className={`px-4 py-2 rounded-lg transition-all ${
            viewMode === 'slider' 
              ? 'bg-orange-400 text-white' 
              : 'glass hover:bg-white/50 dark:hover:bg-slate-700/50'
          }`}
        >
          Slider
        </button>
      </div>

      <div className="glass rounded-2xl p-6">
        {viewMode === 'side-by-side' ? (
          <div className="grid md:grid-cols-2 gap-6">
            <div className="space-y-2">
              <p className="text-center text-sm font-medium text-slate-600 dark:text-slate-400">Original</p>
              <img
                src={originalImage}
                alt="Original"
                className="w-full h-auto rounded-xl shadow-lg"
              />
            </div>
            <div className="space-y-2">
              <p className="text-center text-sm font-medium text-slate-600 dark:text-slate-400">Transformed</p>
              <img
                src={`data:image/jpeg;base64,${transformedImage}`}
                alt="Transformed"
                className="w-full h-auto rounded-xl shadow-lg"
              />
            </div>
          </div>
        ) : (
          <div className="relative overflow-hidden rounded-xl">
            <img
              src={originalImage}
              alt="Original"
              className="w-full h-auto"
            />
            <div 
              className="absolute top-0 left-0 w-full h-full overflow-hidden"
              style={{ clipPath: `inset(0 ${100 - sliderPosition}% 0 0)` }}
            >
              <img
                src={`data:image/jpeg;base64,${transformedImage}`}
                alt="Transformed"
                className="w-full h-auto"
              />
            </div>
            <div 
              className="absolute top-0 bottom-0 w-1 bg-white shadow-xl cursor-ew-resize"
              style={{ left: `${sliderPosition}%` }}
            >
              <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-8 h-8 bg-white rounded-full shadow-lg flex items-center justify-center">
                <svg className="w-4 h-4 text-slate-600 dark:text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 9l4-4 4 4m0 6l-4 4-4-4" />
                </svg>
              </div>
            </div>
            <input
              type="range"
              min="0"
              max="100"
              value={sliderPosition}
              onChange={(e) => setSliderPosition(Number(e.target.value))}
              className="absolute top-0 left-0 w-full h-full opacity-0 cursor-ew-resize"
            />
          </div>
        )}
      </div>

      {processingTime && (
        <div className="text-center text-sm text-slate-600 dark:text-slate-400">
          Processed in {(processingTime / 1000).toFixed(1)} seconds
        </div>
      )}

      <div className="flex justify-center gap-4">
        <button
          onClick={handleDownload}
          className="px-6 py-3 bg-green-500 text-white rounded-xl hover:bg-green-600 dark:hover:bg-green-700 transition-colors font-medium"
        >
          Download Result
        </button>
        <button
          onClick={onReset}
          className="px-6 py-3 glass rounded-xl hover:bg-white/50 dark:hover:bg-slate-700/50 transition-colors font-medium"
        >
          Transform Another
        </button>
      </div>
    </div>
  );
}