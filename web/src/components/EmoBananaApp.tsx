import { useState, useCallback } from 'react';
import EmojiGrid from './EmojiGrid';
import ImageUpload from './ImageUpload';
import TransformResult from './TransformResult';

const API_URL = import.meta.env.PUBLIC_API_URL || 'https://emobanana.guitaripod.workers.dev';

type AppState = 'select-image' | 'select-emoji' | 'processing' | 'result' | 'error';

interface TransformResponse {
  transformed_image: string;
  metadata: {
    processing_time_ms: number;
    model_version: string;
    request_id: string;
  };
}

export default function EmoBananaApp() {
  const [appState, setAppState] = useState<AppState>('select-image');
  const [selectedImage, setSelectedImage] = useState<string | null>(null);
  const [selectedEmoji, setSelectedEmoji] = useState<string | null>(null);
  const [transformedImage, setTransformedImage] = useState<string | null>(null);
  const [processingTime, setProcessingTime] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [requestsRemaining, setRequestsRemaining] = useState<number | null>(null);

  const handleImageSelect = useCallback((imageData: string) => {
    setSelectedImage(imageData);
    setAppState('select-emoji');
    setError(null);
  }, []);

  const handleEmojiSelect = useCallback(async (emoji: string) => {
    if (!selectedImage) return;
    
    setSelectedEmoji(emoji);
    setAppState('processing');
    setError(null);

    try {
      const response = await fetch(`${API_URL}/api/transform`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          image: selectedImage,
          emoji: emoji,
        }),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error?.message || `HTTP error! status: ${response.status}`);
      }

      const data: TransformResponse = await response.json();
      setTransformedImage(data.transformed_image);
      setProcessingTime(data.metadata.processing_time_ms);
      setAppState('result');
    } catch (err) {
      console.error('Transform error:', err);
      setError(err instanceof Error ? err.message : 'Failed to transform image');
      setAppState('error');
      
      if (err instanceof Error && err.message.includes('Rate limit')) {
        setRequestsRemaining(0);
      }
    }
  }, [selectedImage]);

  const handleReset = useCallback(() => {
    setAppState('select-image');
    setSelectedImage(null);
    setSelectedEmoji(null);
    setTransformedImage(null);
    setProcessingTime(null);
    setError(null);
  }, []);

  const handleRetry = useCallback(() => {
    if (selectedEmoji) {
      handleEmojiSelect(selectedEmoji);
    }
  }, [selectedEmoji, handleEmojiSelect]);

  return (
    <div className="min-h-screen py-8 px-4">
      <div className="max-w-7xl mx-auto">
        <header className="text-center mb-12 fade-in">
          <h1 className="text-5xl md:text-6xl font-bold bg-gradient-to-r from-orange-400 to-yellow-400 bg-clip-text text-transparent mb-4">
            EmoBanana
          </h1>
          <p className="text-xl text-slate-600 dark:text-slate-400">
            Transform facial expressions with emoji magic ✨
          </p>
        </header>

        <div className="flex justify-center mb-8">
          <div className="flex items-center gap-4">
            <div className={`flex items-center gap-2 ${appState !== 'select-image' ? 'opacity-50' : ''}`}>
              <div className={`w-8 h-8 rounded-full flex items-center justify-center ${appState === 'select-image' ? 'bg-orange-400 text-white' : 'bg-slate-200 dark:bg-slate-700'}`}>
                1
              </div>
              <span className="text-sm font-medium">Upload Image</span>
            </div>
            <div className="w-8 h-px bg-slate-300 dark:bg-slate-700"></div>
            <div className={`flex items-center gap-2 ${appState !== 'select-emoji' ? 'opacity-50' : ''}`}>
              <div className={`w-8 h-8 rounded-full flex items-center justify-center ${appState === 'select-emoji' ? 'bg-orange-400 text-white' : 'bg-slate-200 dark:bg-slate-700'}`}>
                2
              </div>
              <span className="text-sm font-medium">Select Emoji</span>
            </div>
            <div className="w-8 h-px bg-slate-300 dark:bg-slate-700"></div>
            <div className={`flex items-center gap-2 ${appState !== 'result' ? 'opacity-50' : ''}`}>
              <div className={`w-8 h-8 rounded-full flex items-center justify-center ${appState === 'result' ? 'bg-orange-400 text-white' : 'bg-slate-200 dark:bg-slate-700'}`}>
                3
              </div>
              <span className="text-sm font-medium">Get Result</span>
            </div>
          </div>
        </div>

        <main className="fade-in">
          {requestsRemaining === 0 && (
            <div className="mb-6 p-4 bg-yellow-50 dark:bg-yellow-950/30 border border-yellow-200 dark:border-yellow-800 rounded-xl max-w-2xl mx-auto">
              <p className="text-yellow-800 dark:text-yellow-200 text-center">
                You've reached the daily limit of 5 transformations. Try again tomorrow!
              </p>
            </div>
          )}

          {appState === 'select-image' && (
            <div className="space-y-8">
              <ImageUpload onImageSelect={handleImageSelect} />
            </div>
          )}

          {appState === 'select-emoji' && selectedImage && (
            <div className="space-y-8">
              <div className="max-w-md mx-auto">
                <img
                  src={selectedImage}
                  alt="Selected"
                  className="w-full h-auto max-h-64 object-contain rounded-xl glass p-4 mb-6"
                />
              </div>
              <div className="text-center mb-6">
                <h2 className="text-2xl font-semibold text-slate-700 dark:text-slate-200">
                  Choose an emoji expression
                </h2>
                <p className="text-slate-500 dark:text-slate-400 mt-2">
                  Select the emotion you want to apply to the face
                </p>
              </div>
              <EmojiGrid 
                onEmojiSelect={handleEmojiSelect} 
                selectedEmoji={selectedEmoji}
              />
            </div>
          )}

          {appState === 'processing' && (
            <div className="text-center py-16">
              <div className="inline-block">
                <div className="text-6xl mb-4 animate-pulse-slow">{selectedEmoji}</div>
                <div className="flex items-center justify-center gap-2">
                  <div className="w-3 h-3 bg-orange-400 rounded-full animate-bounce" style={{ animationDelay: '0ms' }}></div>
                  <div className="w-3 h-3 bg-orange-400 rounded-full animate-bounce" style={{ animationDelay: '150ms' }}></div>
                  <div className="w-3 h-3 bg-orange-400 rounded-full animate-bounce" style={{ animationDelay: '300ms' }}></div>
                </div>
                <p className="text-lg text-slate-600 dark:text-slate-400 mt-4">
                  Transforming your image...
                </p>
              </div>
            </div>
          )}

          {appState === 'result' && selectedImage && transformedImage && (
            <TransformResult
              originalImage={selectedImage}
              transformedImage={transformedImage}
              processingTime={processingTime || undefined}
              onReset={handleReset}
            />
          )}

          {appState === 'error' && (
            <div className="max-w-md mx-auto">
              <div className="glass rounded-2xl p-8 text-center">
                <div className="text-6xl mb-4">😕</div>
                <h2 className="text-xl font-semibold text-slate-700 dark:text-slate-200 mb-2">
                  Oops! Something went wrong
                </h2>
                <p className="text-slate-500 dark:text-slate-400 mb-6">{error}</p>
                <div className="flex justify-center gap-4">
                  <button
                    onClick={handleRetry}
                    className="px-6 py-3 bg-orange-400 text-white rounded-xl hover:bg-orange-500 dark:hover:bg-orange-600 transition-colors font-medium"
                  >
                    Try Again
                  </button>
                  <button
                    onClick={handleReset}
                    className="px-6 py-3 glass rounded-xl hover:bg-white/50 dark:hover:bg-slate-700/50 transition-colors font-medium"
                  >
                    Start Over
                  </button>
                </div>
              </div>
            </div>
          )}
        </main>

        <footer className="mt-16 text-center text-sm text-slate-500 dark:text-slate-400">
          <p>Powered by Gemini 2.5 Flash • 5 transformations per day</p>
        </footer>
      </div>
    </div>
  );
}