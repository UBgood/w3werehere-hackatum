'use client'
import DashboardFeature from '@/components/dashboard/dashboard-feature';
import React, { useState } from 'react';
import Button from '@/components/diary/Button';

export default function Page() {
  const handleClick = () => {
    // Handle the button click here
    console.log('Button clicked!');
  };

  return (
    <>
      <DashboardFeature />
      <Button onClick={handleClick} label="Click me" />
    </>
  );
}



