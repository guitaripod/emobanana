import { useState, useCallback } from 'react';
import EmojiGrid from './EmojiGrid';
import ImageUpload from './ImageUpload';
import TransformResult from './TransformResult';
import ErrorDisplay from './ErrorDisplay';

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

interface ErrorResponse {
  error: {
    message: string;
    type: string;
    param?: string;
    code?: string;
    suggestion?: string;
  };
}

export default function EmoBananaApp() {
  const [appState, setAppState] = useState<AppState>('select-image');
  const [selectedImage, setSelectedImage] = useState<string | null>(null);
  const [selectedEmoji, setSelectedEmoji] = useState<string | null>(null);
  const [transformedImage, setTransformedImage] = useState<string | null>(null);
  const [processingTime, setProcessingTime] = useState<number | null>(null);
  const [error, setError] = useState<{
    message: string;
    code?: string;
    type?: string;
    suggestion?: string;
  } | null>(null);
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
        const errorData: ErrorResponse = await response.json();
        const error = new Error(errorData.error?.message || `HTTP error! status: ${response.status}`);
        (error as any).code = errorData.error?.code;
        (error as any).type = errorData.error?.type;
        (error as any).suggestion = errorData.error?.suggestion;
        throw error;
      }

      const data: TransformResponse = await response.json();
      setTransformedImage(data.transformed_image);
      setProcessingTime(data.metadata.processing_time_ms);
      setAppState('result');
    } catch (err) {
      console.error('Transform error:', err);
      if (err instanceof Error) {
        setError({
          message: err.message,
          code: (err as any).code,
          type: (err as any).type,
          suggestion: (err as any).suggestion,
        });

        if (err.message.includes('Rate limit') || (err as any).code === 'rate_limit_exceeded') {
          setRequestsRemaining(0);
        }
      } else {
        setError({
          message: 'Failed to transform image',
        });
      }
      setAppState('error');
    }
  }, [selectedImage]);

  const handleReset = useCallback(() => {
    setAppState('select-image');
    setSelectedImage(null);
    setSelectedEmoji(null);
    setTransformedImage(null);
    setProcessingTime(null);
    setError(null);
    setRequestsRemaining(null);
  }, []);

  const handleRetry = useCallback(() => {
    if (selectedEmoji && selectedImage) {
      // Clear error state before retrying
      setError(null);
      handleEmojiSelect(selectedEmoji);
    }
  }, [selectedEmoji, selectedImage, handleEmojiSelect]);

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
             <ErrorDisplay
               error={error}
               onRetry={handleRetry}
               onReset={handleReset}
             />
           )}
        </main>

        <footer className="mt-16 text-center text-sm text-slate-500 dark:text-slate-400">
          <p>Powered by Gemini 2.5 Flash • 5 transformations per day</p>
          <div className="mt-2 space-x-4">
            <a
              href="/api/docs"
              className="hover:text-slate-700 dark:hover:text-slate-200 transition-colors"
              target="_blank"
              rel="noopener noreferrer"
            >
              API Docs
            </a>
            <span>•</span>
            <a
              href="/api/privacy-policy"
              className="hover:text-slate-700 dark:hover:text-slate-200 transition-colors"
              target="_blank"
              rel="noopener noreferrer"
            >
              Privacy Policy
            </a>
          </div>
        </footer>
      </div>
    </div>
  );
}