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
        return 'Content Filtered';
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
        return 'from-red-400 to-pink-400';
      case 'gemini_api_error':
      case 'gemini_quota_exceeded':
        return 'from-blue-400 to-purple-400';
      default:
        return 'from-slate-400 to-slate-500';
    }
  };

  return (
    <div className="max-w-md mx-auto">
      <div className="glass rounded-2xl p-8 text-center">
        <div className={`text-6xl mb-4 bg-gradient-to-r ${getErrorColor(error.code)} bg-clip-text text-transparent`}>
          {getErrorIcon(error.code)}
        </div>
        <h2 className="text-xl font-semibold text-slate-700 dark:text-slate-200 mb-2">
          {getErrorTitle(error.code)}
        </h2>
        <p className="text-slate-500 dark:text-slate-400 mb-4">
          {error.message}
        </p>
        {error.suggestion && (
          <div className="bg-blue-50 dark:bg-blue-950/30 border border-blue-200 dark:border-blue-800 rounded-xl p-4 mb-6">
            <div className="flex items-start gap-3">
              <div className="text-blue-500 dark:text-blue-400 mt-0.5">💡</div>
              <p className="text-sm text-blue-700 dark:text-blue-300 text-left">
                {error.suggestion}
              </p>
            </div>
          </div>
        )}
        <div className="flex flex-col gap-3">
          <div className="flex justify-center gap-4">
            {onRetry && error.code !== 'rate_limit_exceeded' && error.code !== 'content_filtered' && (
              <button
                onClick={onRetry}
                className="px-6 py-3 bg-gradient-to-r from-orange-400 to-yellow-400 text-white rounded-xl hover:from-orange-500 hover:to-yellow-500 dark:hover:from-orange-600 dark:hover:to-yellow-600 transition-all font-medium shadow-lg hover:shadow-xl"
              >
                Try Again
              </button>
            )}
            {onReset && (
              <button
                onClick={onReset}
                className="px-6 py-3 glass rounded-xl hover:bg-white/50 dark:hover:bg-slate-700/50 transition-colors font-medium"
              >
                {error.code === 'rate_limit_exceeded' ? 'Try Tomorrow' : 'Start Over'}
              </button>
            )}
          </div>
          {error.code === 'rate_limit_exceeded' && (
            <p className="text-xs text-slate-400 dark:text-slate-500 text-center">
              Rate limits reset daily at midnight UTC
            </p>
          )}
        </div>
      </div>
    </div>
  );
};

export default ErrorDisplay;