import React from 'react';

interface ErrorInfo {
  message: string;
  code?: string;
  type?: string;
  suggestion?: string;
}

interface ErrorDisplayProps {
  error: ErrorInfo | null;
  onRetry?: () => void;
  onReset?: () => void;
}

const ErrorDisplay: React.FC<ErrorDisplayProps> = ({ error, onRetry, onReset }) => {
  if (!error) return null;

  const getErrorIcon = (code?: string) => {
    switch (code) {
      case 'rate_limit_exceeded':
        return '⏰';
      case 'invalid_image_format':
      case 'unsupported_image_type':
      case 'image_too_large':
        return '📷';
      case 'gemini_api_error':
      case 'gemini_quota_exceeded':
        return '🤖';
      case 'content_filtered':
        return '🚫';
      case 'no_faces_detected':
        return '👤';
      case 'transformation_failed':
        return '🎭';
      case 'ai_timeout':
        return '⏳';
      default:
        return '😕';
    }
  };

  const getErrorTitle = (code?: string) => {
    switch (code) {
      case 'rate_limit_exceeded':
        return 'Rate Limit Reached';
      case 'invalid_image_format':
        return 'Invalid Image Format';
      case 'unsupported_image_type':
        return 'Unsupported Image Type';
      case 'image_too_large':
        return 'Image Too Large';
      case 'gemini_api_error':
        return 'AI Service Unavailable';
      case 'gemini_quota_exceeded':
        return 'AI Service Busy';
      case 'content_filtered':
        return 'Content Not Allowed';
      case 'no_faces_detected':
        return 'No Faces Detected';
      case 'transformation_failed':
        return 'Transformation Failed';
      case 'ai_timeout':
        return 'Request Timeout';
      default:
        return 'Something Went Wrong';
    }
  };

  const getErrorColor = (code?: string) => {
    switch (code) {
      case 'rate_limit_exceeded':
        return 'from-yellow-400 to-orange-400';
      case 'content_filtered':
        return 'from-red-500 to-red-600';
      case 'gemini_api_error':
      case 'gemini_quota_exceeded':
        return 'from-blue-400 to-purple-400';
      default:
        return 'from-slate-400 to-slate-500';
    }
  };

  const getErrorBgColor = (code?: string) => {
    switch (code) {
      case 'rate_limit_exceeded':
        return 'bg-yellow-50 dark:bg-yellow-950/20 border-yellow-200 dark:border-yellow-800';
      case 'content_filtered':
        return 'bg-red-50 dark:bg-red-950/20 border-red-200 dark:border-red-800';
      case 'gemini_api_error':
      case 'gemini_quota_exceeded':
        return 'bg-blue-50 dark:bg-blue-950/20 border-blue-200 dark:border-blue-800';
      default:
        return 'bg-slate-50 dark:bg-slate-950/20 border-slate-200 dark:border-slate-800';
    }
  };

  const isContentFiltered = error.code === 'content_filtered';
  const isRateLimited = error.code === 'rate_limit_exceeded';

  return (
    <div className="max-w-lg mx-auto">
      <div className={`${getErrorBgColor(error.code)} border-2 rounded-3xl p-8 text-center shadow-xl backdrop-blur-sm`}>
        {/* Large Error Icon */}
        <div className={`text-8xl mb-6 bg-gradient-to-r ${getErrorColor(error.code)} bg-clip-text text-transparent animate-pulse`}>
          {getErrorIcon(error.code)}
        </div>

        {/* Error Title - More Prominent */}
        <h1 className="text-2xl font-bold text-slate-800 dark:text-slate-100 mb-3">
          {getErrorTitle(error.code)}
        </h1>

        {/* Error Message - Much More Prominent */}
        <div className="mb-6">
          <p className={`text-lg font-medium ${
            isContentFiltered
              ? 'text-red-700 dark:text-red-300'
              : 'text-slate-700 dark:text-slate-300'
          }`}>
            {error.message}
          </p>
        </div>

        {/* Suggestion Box - More Integrated */}
        {error.suggestion && (
          <div className={`${
            isContentFiltered
              ? 'bg-red-100 dark:bg-red-950/40 border-red-300 dark:border-red-700'
              : 'bg-blue-100 dark:bg-blue-950/40 border-blue-300 dark:border-blue-700'
          } border-2 rounded-2xl p-5 mb-8 shadow-inner`}>
            <div className="flex items-start gap-4">
              <div className={`text-2xl ${isContentFiltered ? 'text-red-500' : 'text-blue-500'}`}>
                💡
              </div>
              <div className="text-left">
                <p className={`text-base font-semibold mb-1 ${
                  isContentFiltered
                    ? 'text-red-800 dark:text-red-200'
                    : 'text-blue-800 dark:text-blue-200'
                }`}>
                  {isContentFiltered ? 'Try this instead:' : 'What you can do:'}
                </p>
                <p className={`text-sm leading-relaxed ${
                  isContentFiltered
                    ? 'text-red-700 dark:text-red-300'
                    : 'text-blue-700 dark:text-blue-300'
                }`}>
                  {error.suggestion}
                </p>
              </div>
            </div>
          </div>
        )}

        {/* Action Buttons - More Prominent */}
        <div className="flex flex-col gap-4">
          <div className="flex justify-center gap-4">
            {onRetry && !isContentFiltered && !isRateLimited && (
              <button
                onClick={onRetry}
                className="px-8 py-4 bg-gradient-to-r from-orange-500 to-yellow-500 text-white rounded-2xl hover:from-orange-600 hover:to-yellow-600 dark:hover:from-orange-700 dark:hover:to-yellow-700 transition-all font-semibold shadow-xl hover:shadow-2xl transform hover:scale-105 text-lg"
              >
                🔄 Try Again
              </button>
            )}
            {onReset && (
              <button
                onClick={onReset}
                className={`px-8 py-4 rounded-2xl transition-all font-semibold shadow-xl hover:shadow-2xl transform hover:scale-105 text-lg ${
                  isContentFiltered
                    ? 'bg-red-500 hover:bg-red-600 text-white'
                    : 'glass hover:bg-white/60 dark:hover:bg-slate-700/60 text-slate-700 dark:text-slate-200'
                }`}
              >
                {isRateLimited ? '⏰ Try Tomorrow' : '🏠 Start Over'}
              </button>
            )}
          </div>

          {/* Additional Info for Rate Limits */}
          {isRateLimited && (
            <div className="mt-4 p-4 bg-yellow-100 dark:bg-yellow-950/40 border border-yellow-300 dark:border-yellow-700 rounded-xl">
              <p className="text-sm text-yellow-800 dark:text-yellow-200 font-medium">
                ⏰ Rate limits reset daily at midnight UTC
              </p>
            </div>
          )}

          {/* Special Message for Content Filtered */}
          {isContentFiltered && (
            <div className="mt-4 p-4 bg-red-100 dark:bg-red-950/40 border border-red-300 dark:border-red-700 rounded-xl">
              <p className="text-sm text-red-800 dark:text-red-200 font-medium">
                🔒 Google's Gemini AI service enforces their own content policies
              </p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default ErrorDisplay;